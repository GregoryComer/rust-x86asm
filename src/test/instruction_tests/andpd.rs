use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn andpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 224], OperandSize::Dword)
}

fn andpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 20, 246], OperandSize::Dword)
}

fn andpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 252], OperandSize::Qword)
}

fn andpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDPD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RDI, 1749963238, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 84, 191, 230, 81, 78, 104], OperandSize::Qword)
}

