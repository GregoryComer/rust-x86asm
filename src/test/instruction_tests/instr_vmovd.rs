use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 249], OperandSize::Dword)
}

#[test]
fn vmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(ECX, 1785703150, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 137, 238, 170, 111, 106], OperandSize::Dword)
}

#[test]
fn vmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 197], OperandSize::Qword)
}

#[test]
fn vmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RBX, 71969591, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 139, 55, 43, 74, 4], OperandSize::Qword)
}

#[test]
fn vmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 247], OperandSize::Dword)
}

#[test]
fn vmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 110, 44, 81], OperandSize::Dword)
}

#[test]
fn vmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 8, 110, 217], OperandSize::Qword)
}

#[test]
fn vmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 110, 60, 185], OperandSize::Qword)
}

#[test]
fn vmovd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 214], OperandSize::Dword)
}

#[test]
fn vmovd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectDisplaced(EDX, 1409154045, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 186, 253, 251, 253, 83], OperandSize::Dword)
}

#[test]
fn vmovd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 220], OperandSize::Qword)
}

#[test]
fn vmovd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Eight, 1948757293, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 132, 211, 45, 173, 39, 116], OperandSize::Qword)
}

#[test]
fn vmovd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 195], OperandSize::Dword)
}

#[test]
fn vmovd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectDisplaced(EDX, 1998166317, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 126, 130, 45, 153, 25, 119], OperandSize::Dword)
}

#[test]
fn vmovd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 8, 126, 231], OperandSize::Qword)
}

#[test]
fn vmovd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVD, operand1: Some(IndirectScaledIndexed(RDX, RAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 125, 8, 126, 12, 130], OperandSize::Qword)
}

