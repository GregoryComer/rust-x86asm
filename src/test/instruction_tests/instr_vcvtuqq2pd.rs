use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtuqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 137, 122, 241], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 239019015, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 137, 122, 60, 93, 7, 36, 63, 14], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 65, 254, 138, 122, 208], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1389529357, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 254, 141, 122, 20, 133, 13, 137, 210, 82], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 174, 122, 240], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(ECX, 556354582, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 170, 122, 145, 22, 76, 41, 33], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 254, 174, 122, 201], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 171, 122, 54], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 191, 122, 236], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectDisplaced(EDI, 351613559, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 203, 122, 191, 119, 50, 245, 20], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 254, 222, 122, 194], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1664718812, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 254, 201, 122, 28, 69, 220, 151, 57, 99], OperandSize::Qword)
}

