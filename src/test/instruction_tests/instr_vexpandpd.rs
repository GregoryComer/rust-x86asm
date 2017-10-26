use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexpandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 136, 203], OperandSize::Dword)
}

#[test]
fn vexpandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 268297940, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 137, 136, 164, 131, 212, 230, 253, 15], OperandSize::Dword)
}

#[test]
fn vexpandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 253, 138, 136, 249], OperandSize::Qword)
}

#[test]
fn vexpandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 652269624, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 136, 156, 209, 56, 216, 224, 38], OperandSize::Qword)
}

#[test]
fn vexpandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 136, 220], OperandSize::Dword)
}

#[test]
fn vexpandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 384064862, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 136, 140, 83, 94, 93, 228, 22], OperandSize::Dword)
}

#[test]
fn vexpandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 253, 169, 136, 204], OperandSize::Qword)
}

#[test]
fn vexpandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 253, 175, 136, 12, 198], OperandSize::Qword)
}

#[test]
fn vexpandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 136, 252], OperandSize::Dword)
}

#[test]
fn vexpandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 851272237, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 136, 188, 183, 45, 98, 189, 50], OperandSize::Dword)
}

#[test]
fn vexpandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 253, 203, 136, 192], OperandSize::Qword)
}

#[test]
fn vexpandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM30)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 299042531, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 205, 136, 52, 157, 227, 6, 211, 17], OperandSize::Qword)
}

