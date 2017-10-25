use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM0)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 195], OperandSize::Dword)
}

#[test]
fn movd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EDI, 1827110695, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 151, 39, 127, 231, 108], OperandSize::Dword)
}

#[test]
fn movd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM7)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 249], OperandSize::Qword)
}

#[test]
fn movd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(RAX, 1818889644, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 110, 136, 172, 13, 106, 108], OperandSize::Qword)
}

#[test]
fn movd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 255], OperandSize::Dword)
}

#[test]
fn movd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1084799605, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 44, 181, 117, 186, 168, 64], OperandSize::Dword)
}

#[test]
fn movd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 237], OperandSize::Qword)
}

#[test]
fn movd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 110, 4, 86], OperandSize::Qword)
}

#[test]
fn movd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(ESP)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 196], OperandSize::Dword)
}

#[test]
fn movd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectDisplaced(EDX, 284896182, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 186, 182, 43, 251, 16], OperandSize::Dword)
}

#[test]
fn movd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBX)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 203], OperandSize::Qword)
}

#[test]
fn movd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 126, 20, 186], OperandSize::Qword)
}

#[test]
fn movd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 215], OperandSize::Dword)
}

#[test]
fn movd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 27], OperandSize::Dword)
}

#[test]
fn movd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 205], OperandSize::Qword)
}

#[test]
fn movd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Two, 220110881, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 126, 140, 115, 33, 160, 30, 13], OperandSize::Qword)
}

