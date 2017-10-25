use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 195], OperandSize::Dword)
}

#[test]
fn vmovsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 16, 215], OperandSize::Qword)
}

#[test]
fn vmovsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 52, 82], OperandSize::Dword)
}

#[test]
fn vmovsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 16], OperandSize::Qword)
}

#[test]
fn vmovsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 207, 139, 16, 194], OperandSize::Dword)
}

#[test]
fn vmovsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 151, 130, 16, 232], OperandSize::Qword)
}

#[test]
fn vmovsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 2005672865, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 143, 16, 12, 117, 161, 35, 140, 119], OperandSize::Dword)
}

#[test]
fn vmovsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM18)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 255, 141, 16, 19], OperandSize::Qword)
}

#[test]
fn vmovsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 16, 221], OperandSize::Dword)
}

#[test]
fn vmovsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 203, 16, 248], OperandSize::Qword)
}

#[test]
fn vmovsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexed(ECX, EDX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 28, 145], OperandSize::Dword)
}

#[test]
fn vmovsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 28, 211], OperandSize::Qword)
}

#[test]
fn vmovsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 239, 141, 16, 208], OperandSize::Dword)
}

#[test]
fn vmovsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 143, 134, 16, 254], OperandSize::Qword)
}

#[test]
fn vmovsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectDisplaced(ESI, 1724839975, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 150, 39, 248, 206, 102], OperandSize::Dword)
}

#[test]
fn vmovsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Two, 1836325423, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 255, 8, 17, 132, 81, 47, 26, 116, 109], OperandSize::Qword)
}

