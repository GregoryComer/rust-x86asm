use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 58, 252], OperandSize::Dword)
}

#[test]
fn vpminuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1844774703, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 58, 36, 253, 47, 7, 245, 109], OperandSize::Dword)
}

#[test]
fn vpminuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 58, 255], OperandSize::Qword)
}

#[test]
fn vpminuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 975230390, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 58, 140, 142, 182, 213, 32, 58], OperandSize::Qword)
}

#[test]
fn vpminuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 58, 230], OperandSize::Dword)
}

#[test]
fn vpminuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 58, 34], OperandSize::Dword)
}

#[test]
fn vpminuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 58, 199], OperandSize::Qword)
}

#[test]
fn vpminuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 58, 56], OperandSize::Qword)
}

#[test]
fn vpminuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 58, 199], OperandSize::Dword)
}

#[test]
fn vpminuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 966743041, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 137, 58, 164, 143, 1, 84, 159, 57], OperandSize::Dword)
}

#[test]
fn vpminuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 109, 129, 58, 241], OperandSize::Qword)
}

#[test]
fn vpminuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 45, 138, 58, 1], OperandSize::Qword)
}

#[test]
fn vpminuw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 171, 58, 242], OperandSize::Dword)
}

#[test]
fn vpminuw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EAX, 1507873003, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 173, 58, 152, 235, 80, 224, 89], OperandSize::Dword)
}

#[test]
fn vpminuw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 37, 161, 58, 248], OperandSize::Qword)
}

#[test]
fn vpminuw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 61, 170, 58, 0], OperandSize::Qword)
}

#[test]
fn vpminuw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 202, 58, 237], OperandSize::Dword)
}

#[test]
fn vpminuw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 1029910461, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 201, 58, 148, 82, 189, 47, 99, 61], OperandSize::Dword)
}

#[test]
fn vpminuw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 61, 194, 58, 230], OperandSize::Qword)
}

#[test]
fn vpminuw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUW, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RSI, 1966673213, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 117, 193, 58, 190, 61, 13, 57, 117], OperandSize::Qword)
}

