use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM3)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 222], OperandSize::Dword)
}

#[test]
fn movd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 2123776298, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 4, 141, 42, 65, 150, 126], OperandSize::Dword)
}

#[test]
fn movd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM4)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 231], OperandSize::Qword)
}

#[test]
fn movd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 621500649, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 60, 197, 233, 88, 11, 37], OperandSize::Qword)
}

#[test]
fn movd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 241], OperandSize::Dword)
}

#[test]
fn movd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 47], OperandSize::Dword)
}

#[test]
fn movd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 212], OperandSize::Qword)
}

#[test]
fn movd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 4, 120], OperandSize::Qword)
}

#[test]
fn movd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 255], OperandSize::Dword)
}

#[test]
fn movd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(EDX, 1112101486, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 138, 110, 82, 73, 66], OperandSize::Dword)
}

#[test]
fn movd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBX)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 251], OperandSize::Qword)
}

#[test]
fn movd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 4, 222], OperandSize::Qword)
}

#[test]
fn movd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 221], OperandSize::Dword)
}

#[test]
fn movd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 363149858, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 4, 221, 34, 58, 165, 21], OperandSize::Dword)
}

#[test]
fn movd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 238], OperandSize::Qword)
}

#[test]
fn movd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(RCX, 1428539222, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 153, 86, 199, 37, 85], OperandSize::Qword)
}

