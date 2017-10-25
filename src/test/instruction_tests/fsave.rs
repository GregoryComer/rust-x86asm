use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fsave_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectDisplaced(BX, 23111, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 183, 71, 90], OperandSize::Word)
}

fn fsave_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 52, 242], OperandSize::Dword)
}

fn fsave_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSAVE, operand1: Some(IndirectDisplaced(RDI, 1300603616, Some(OperandSize::Unsized), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 221, 183, 224, 162, 133, 77], OperandSize::Qword)
}

