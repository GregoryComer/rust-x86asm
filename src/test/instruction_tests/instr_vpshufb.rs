use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpshufb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 0, 205], OperandSize::Dword)
}

#[test]
fn vpshufb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 0, 9], OperandSize::Dword)
}

#[test]
fn vpshufb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 0, 192], OperandSize::Qword)
}

#[test]
fn vpshufb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(RAX, 482037482, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 0, 176, 234, 78, 187, 28], OperandSize::Qword)
}

#[test]
fn vpshufb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 0, 253], OperandSize::Dword)
}

#[test]
fn vpshufb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 345070328, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 0, 20, 221, 248, 90, 145, 20], OperandSize::Dword)
}

#[test]
fn vpshufb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 0, 228], OperandSize::Qword)
}

#[test]
fn vpshufb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(RSI, 1358137585, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 0, 134, 241, 136, 243, 80], OperandSize::Qword)
}

#[test]
fn vpshufb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 0, 226], OperandSize::Dword)
}

#[test]
fn vpshufb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Four, 419248895, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 139, 0, 132, 183, 255, 58, 253, 24], OperandSize::Dword)
}

#[test]
fn vpshufb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 21, 129, 0, 192], OperandSize::Qword)
}

#[test]
fn vpshufb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 328740224, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 117, 130, 0, 132, 192, 128, 45, 152, 19], OperandSize::Qword)
}

#[test]
fn vpshufb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 170, 0, 236], OperandSize::Dword)
}

#[test]
fn vpshufb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1830858825, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 0, 188, 219, 73, 176, 32, 109], OperandSize::Dword)
}

#[test]
fn vpshufb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 101, 169, 0, 243], OperandSize::Qword)
}

#[test]
fn vpshufb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 109, 166, 0, 23], OperandSize::Qword)
}

#[test]
fn vpshufb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 205, 0, 242], OperandSize::Dword)
}

#[test]
fn vpshufb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1855041898, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 203, 0, 180, 192, 106, 177, 145, 110], OperandSize::Dword)
}

#[test]
fn vpshufb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 101, 207, 0, 219], OperandSize::Qword)
}

#[test]
fn vpshufb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFB, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 1142741343, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 61, 204, 0, 148, 83, 95, 217, 28, 68], OperandSize::Qword)
}

