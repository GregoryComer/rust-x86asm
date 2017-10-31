use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 16, 244], OperandSize::Dword)
}

#[test]
fn vmovss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 16, 248], OperandSize::Qword)
}

#[test]
fn vmovss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 327446723, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 36, 197, 195, 112, 132, 19], OperandSize::Dword)
}

#[test]
fn vmovss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 30], OperandSize::Qword)
}

#[test]
fn vmovss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 16, 241], OperandSize::Dword)
}

#[test]
fn vmovss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 86, 130, 16, 250], OperandSize::Qword)
}

#[test]
fn vmovss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1728904471, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 16, 36, 141, 23, 253, 12, 103], OperandSize::Dword)
}

#[test]
fn vmovss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 126, 139, 16, 44, 150], OperandSize::Qword)
}

#[test]
fn vmovss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 229], OperandSize::Dword)
}

#[test]
fn vmovss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 16, 206], OperandSize::Qword)
}

#[test]
fn vmovss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 905971672, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 172, 150, 216, 7, 0, 54], OperandSize::Dword)
}

#[test]
fn vmovss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 7], OperandSize::Qword)
}

#[test]
fn vmovss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 110, 142, 16, 240], OperandSize::Dword)
}

#[test]
fn vmovss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 70, 140, 16, 239], OperandSize::Qword)
}

#[test]
fn vmovss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 28, 218], OperandSize::Dword)
}

#[test]
fn vmovss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 126, 8, 17, 52, 248], OperandSize::Qword)
}

