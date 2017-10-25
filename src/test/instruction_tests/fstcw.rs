use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn fstcw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(IndirectDisplaced(BX, 69, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 127, 69], OperandSize::Word)
}

fn fstcw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1506646940, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 60, 69, 156, 155, 205, 89], OperandSize::Dword)
}

fn fstcw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FSTCW, operand1: Some(IndirectScaledDisplaced(RBX, Four, 433830290, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[155, 217, 60, 157, 146, 185, 219, 25], OperandSize::Qword)
}

