use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpacksswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 99, 207], OperandSize::Dword)
}

#[test]
fn vpacksswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 181131953, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 99, 44, 205, 177, 218, 203, 10], OperandSize::Dword)
}

#[test]
fn vpacksswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 99, 200], OperandSize::Qword)
}

#[test]
fn vpacksswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 1729097807, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 99, 156, 219, 79, 240, 15, 103], OperandSize::Qword)
}

#[test]
fn vpacksswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 99, 250], OperandSize::Dword)
}

#[test]
fn vpacksswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1687700454, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 99, 164, 119, 230, 67, 152, 100], OperandSize::Dword)
}

#[test]
fn vpacksswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 99, 241], OperandSize::Qword)
}

#[test]
fn vpacksswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1172820892, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 99, 172, 201, 156, 211, 231, 69], OperandSize::Qword)
}

#[test]
fn vpacksswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 69, 139, 99, 254], OperandSize::Dword)
}

#[test]
fn vpacksswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 137, 99, 20, 193], OperandSize::Dword)
}

#[test]
fn vpacksswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 93, 140, 99, 237], OperandSize::Qword)
}

#[test]
fn vpacksswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 69, 139, 99, 52, 65], OperandSize::Qword)
}

#[test]
fn vpacksswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 171, 99, 235], OperandSize::Dword)
}

#[test]
fn vpacksswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 175, 99, 44, 127], OperandSize::Dword)
}

#[test]
fn vpacksswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 85, 170, 99, 195], OperandSize::Qword)
}

#[test]
fn vpacksswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 497301833, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 167, 99, 4, 93, 73, 57, 164, 29], OperandSize::Qword)
}

#[test]
fn vpacksswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 203, 99, 230], OperandSize::Dword)
}

#[test]
fn vpacksswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 93, 201, 99, 14], OperandSize::Dword)
}

#[test]
fn vpacksswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 93, 204, 99, 247], OperandSize::Qword)
}

#[test]
fn vpacksswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKSSWB, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 125, 207, 99, 0], OperandSize::Qword)
}

