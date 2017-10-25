use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fnstenv_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 239, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 176, 239, 0], OperandSize::Word)
}

fn fnstenv_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectDisplaced(EAX, 1750079910, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 176, 166, 25, 80, 104], OperandSize::Dword)
}

fn fnstenv_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTENV, operand1: Some(IndirectScaledIndexed(RSI, RDI, Two, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 52, 126], OperandSize::Qword)
}

