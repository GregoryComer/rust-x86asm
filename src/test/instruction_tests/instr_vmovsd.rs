use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 16, 213], OperandSize::Dword)
}

#[test]
fn vmovsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 16, 253], OperandSize::Qword)
}

#[test]
fn vmovsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ESI, 1696903216, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 182, 48, 176, 36, 101], OperandSize::Dword)
}

#[test]
fn vmovsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 27], OperandSize::Qword)
}

#[test]
fn vmovsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 199, 141, 16, 255], OperandSize::Dword)
}

#[test]
fn vmovsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 151, 138, 16, 197], OperandSize::Qword)
}

#[test]
fn vmovsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EDX, 164184411, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 16, 186, 91, 65, 201, 9], OperandSize::Dword)
}

#[test]
fn vmovsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1130553919, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 143, 16, 188, 138, 63, 226, 98, 67], OperandSize::Qword)
}

#[test]
fn vmovsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 222], OperandSize::Dword)
}

#[test]
fn vmovsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 227, 16, 236], OperandSize::Qword)
}

#[test]
fn vmovsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 579345656, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 156, 72, 248, 28, 136, 34], OperandSize::Dword)
}

#[test]
fn vmovsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledDisplaced(RAX, Four, 1191171795, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 52, 133, 211, 214, 255, 70], OperandSize::Qword)
}

#[test]
fn vmovsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 239, 137, 16, 212], OperandSize::Dword)
}

#[test]
fn vmovsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 183, 137, 16, 254], OperandSize::Qword)
}

#[test]
fn vmovsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 36, 243], OperandSize::Dword)
}

#[test]
fn vmovsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledDisplaced(RDX, Four, 362633469, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 255, 8, 17, 36, 149, 253, 88, 157, 21], OperandSize::Qword)
}

