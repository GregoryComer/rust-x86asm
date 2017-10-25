use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn andnpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 213], OperandSize::Dword)
}

fn andnpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(ESI, 1723143857, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 158, 177, 22, 181, 102], OperandSize::Dword)
}

fn andnpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 231], OperandSize::Qword)
}

fn andnpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ANDNPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDI, 198117800, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 85, 175, 168, 9, 207, 11], OperandSize::Qword)
}

