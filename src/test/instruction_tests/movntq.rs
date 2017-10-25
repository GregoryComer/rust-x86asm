use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movntq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(Indirect(BX, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 23], OperandSize::Word)
}

fn movntq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1368778483, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 60, 189, 243, 230, 149, 81], OperandSize::Dword)
}

fn movntq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTQ, operand1: Some(IndirectScaledDisplaced(RCX, Four, 59929081, Some(OperandSize::Qword), None)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 231, 36, 141, 249, 113, 146, 3], OperandSize::Qword)
}

