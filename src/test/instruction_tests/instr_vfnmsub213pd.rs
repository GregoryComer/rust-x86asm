use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 174, 201], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 428223398, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 174, 132, 65, 166, 43, 134, 25], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 174, 233], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Two, 88563005, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 174, 156, 122, 61, 93, 71, 5], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 174, 255], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EBX, 2114819039, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 174, 187, 223, 147, 13, 126], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 174, 204], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 174, 28, 219], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 174, 253], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 24510933, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 174, 36, 93, 213, 1, 118, 1], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 443958270, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 155, 174, 164, 118, 254, 67, 118, 26], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 165, 130, 174, 205], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectDisplaced(RAX, 308342742, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 157, 137, 174, 136, 214, 239, 96, 18], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 2091562385, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 213, 156, 174, 52, 181, 145, 181, 170, 124], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 174, 230], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 198525477, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 174, 160, 37, 66, 213, 11], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 188, 174, 44, 223], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 221, 171, 174, 253], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 807526507, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 213, 172, 174, 36, 69, 107, 224, 33, 48], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1083919143, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 141, 185, 174, 20, 117, 39, 75, 155, 64], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 250, 174, 219], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 204, 174, 52, 206], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 222, 174, 24], OperandSize::Dword)
}

#[test]
fn vfnmsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 165, 145, 174, 221], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1040140545, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 149, 202, 174, 44, 93, 1, 73, 255, 61], OperandSize::Qword)
}

#[test]
fn vfnmsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB213PD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 173, 218, 174, 19], OperandSize::Qword)
}

