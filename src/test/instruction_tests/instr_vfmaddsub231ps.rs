use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 182, 235], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 807333536, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 182, 164, 66, 160, 238, 30, 48], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 182, 240], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 182, 36, 210], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 182, 223], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 182, 9], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 182, 194], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 182, 43], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 182, 224], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 140, 182, 12, 122], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1161978543, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 159, 182, 132, 90, 175, 98, 66, 69], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 109, 131, 182, 197], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1521614476, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 37, 137, 182, 4, 117, 140, 254, 177, 90], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1602288722, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 13, 158, 182, 12, 93, 82, 252, 128, 95], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 182, 241], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 988936553, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 169, 182, 36, 189, 105, 249, 241, 58], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1514340119, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 187, 182, 20, 77, 23, 255, 66, 90], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 117, 166, 182, 193], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM23)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 69, 162, 182, 48], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RBX, 417672949, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 53, 182, 182, 155, 245, 46, 229, 24], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 222, 182, 220], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 833631967, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 182, 12, 77, 223, 54, 176, 49], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1529010620, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 217, 182, 28, 69, 188, 217, 34, 91], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 125, 177, 182, 202], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 117, 196, 182, 41], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 101, 211, 182, 52, 138], OperandSize::Qword)
}

