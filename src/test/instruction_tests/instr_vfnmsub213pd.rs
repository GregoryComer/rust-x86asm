use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 174, 225], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 225, 174, 44, 194], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 174, 202], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 174, 52, 134], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 174, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 2135996001, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 174, 36, 125, 97, 182, 80, 127], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 174, 202], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1857515661, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 174, 148, 190, 141, 112, 183, 110], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 174, 205], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(EBX, 49544380, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 142, 174, 155, 188, 252, 243, 2], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1864145410, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 154, 174, 188, 158, 2, 154, 28, 111], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 229, 133, 174, 220], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 245, 141, 174, 52, 202], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1168044509, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 245, 158, 174, 4, 125, 221, 241, 158, 69], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 174, 196], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 169, 174, 19], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ESI, 1868927818, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 185, 174, 158, 74, 147, 101, 111], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 165, 163, 174, 207], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 758256360, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 165, 165, 174, 164, 66, 232, 18, 50, 45], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1452320425, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 189, 182, 174, 4, 205, 169, 166, 144, 86], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 188, 174, 235], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1548817664, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 174, 156, 150, 0, 21, 81, 92], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 105098968, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 220, 174, 60, 253, 216, 174, 67, 6], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 253, 214, 174, 254], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 390392149, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 197, 204, 174, 148, 215, 85, 233, 68, 23], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 165, 215, 174, 40], OperandSize::Qword)
}

