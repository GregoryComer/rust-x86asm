use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 16, 209], OperandSize::Dword)
}

#[test]
fn vmovss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 16, 225], OperandSize::Qword)
}

#[test]
fn vmovss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 971916399, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 60, 205, 111, 68, 238, 57], OperandSize::Dword)
}

#[test]
fn vmovss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 20, 139], OperandSize::Qword)
}

#[test]
fn vmovss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 16, 227], OperandSize::Dword)
}

#[test]
fn vmovss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 46, 143, 16, 211], OperandSize::Qword)
}

#[test]
fn vmovss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ESI, 445845543, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 16, 190, 39, 16, 147, 26], OperandSize::Dword)
}

#[test]
fn vmovss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM22)), operand2: Some(IndirectDisplaced(RDX, 1785671626, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 126, 141, 16, 178, 202, 47, 111, 106], OperandSize::Qword)
}

#[test]
fn vmovss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 218, 16, 215], OperandSize::Dword)
}

#[test]
fn vmovss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 16, 245], OperandSize::Qword)
}

#[test]
fn vmovss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectDisplaced(EDI, 1105729756, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 159, 220, 24, 232, 65], OperandSize::Dword)
}

#[test]
fn vmovss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectDisplaced(RDI, 496555439, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 143, 175, 213, 152, 29], OperandSize::Qword)
}

#[test]
fn vmovss_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 118, 141, 16, 241], OperandSize::Dword)
}

#[test]
fn vmovss_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 17, 94, 131, 16, 243], OperandSize::Qword)
}

#[test]
fn vmovss_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 1285154726, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 17, 36, 253, 166, 231, 153, 76], OperandSize::Dword)
}

#[test]
fn vmovss_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 142750632, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 126, 8, 17, 188, 183, 168, 51, 130, 8], OperandSize::Qword)
}

