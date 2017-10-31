use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 231], OperandSize::Dword)
}

#[test]
fn vmovsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 252], OperandSize::Qword)
}

#[test]
fn vmovsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EBX, 1558140783, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 139, 111, 87, 223, 92], OperandSize::Dword)
}

#[test]
fn vmovsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Four, 1820426427, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 16, 180, 187, 187, 128, 129, 108], OperandSize::Qword)
}

#[test]
fn vmovsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 231, 140, 16, 209], OperandSize::Dword)
}

#[test]
fn vmovsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 255, 143, 16, 199], OperandSize::Qword)
}

#[test]
fn vmovsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 143, 16, 40], OperandSize::Dword)
}

#[test]
fn vmovsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 255, 138, 16, 20, 183], OperandSize::Qword)
}

#[test]
fn vmovsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 16, 199], OperandSize::Dword)
}

#[test]
fn vmovsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 16, 197], OperandSize::Qword)
}

#[test]
fn vmovsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 71595681, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 172, 184, 161, 118, 68, 4], OperandSize::Dword)
}

#[test]
fn vmovsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1773365943, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 28, 197, 183, 106, 179, 105], OperandSize::Qword)
}

#[test]
fn vmovsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 199, 141, 16, 236], OperandSize::Dword)
}

#[test]
fn vmovsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM25)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 81, 183, 130, 16, 238], OperandSize::Qword)
}

#[test]
fn vmovsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 17, 12, 144], OperandSize::Dword)
}

#[test]
fn vmovsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVSD, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 255, 8, 17, 55], OperandSize::Qword)
}

