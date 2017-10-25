use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fnstcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 0, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 56], OperandSize::Word)
}

fn fnstcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(IndirectDisplaced(ECX, 2035059813, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 185, 101, 140, 76, 121], OperandSize::Dword)
}

fn fnstcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FNSTCW, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[217, 62], OperandSize::Qword)
}

