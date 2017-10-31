use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtuqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 122, 232], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 500861511, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 138, 122, 148, 64, 71, 138, 218, 29], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 137, 122, 197], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectDisplaced(RSI, 1193666439, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 254, 138, 122, 190, 135, 231, 37, 71], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 174, 122, 220], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 169, 122, 12, 119], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 254, 170, 122, 240], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1260003422, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 172, 122, 140, 87, 94, 32, 26, 75], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 157, 122, 194], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 205, 122, 63], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 254, 223, 122, 217], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 524700245, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 254, 202, 122, 60, 205, 85, 74, 70, 31], OperandSize::Qword)
}

