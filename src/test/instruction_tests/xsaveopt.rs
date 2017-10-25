use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn xsaveopt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1733525072, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 180, 219, 80, 126, 83, 103], OperandSize::Dword)
}

fn xsaveopt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XSAVEOPT, operand1: Some(IndirectDisplaced(RSI, 1470952694, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 174, 182, 246, 244, 172, 87], OperandSize::Qword)
}

