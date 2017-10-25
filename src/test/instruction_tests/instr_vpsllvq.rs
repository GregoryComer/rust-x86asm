use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsllvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 71, 202], OperandSize::Dword)
}

#[test]
fn vpsllvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 71, 44, 147], OperandSize::Dword)
}

#[test]
fn vpsllvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 71, 225], OperandSize::Qword)
}

#[test]
fn vpsllvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 71, 60, 135], OperandSize::Qword)
}

#[test]
fn vpsllvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 71, 200], OperandSize::Dword)
}

#[test]
fn vpsllvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EAX, 1380573116, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 71, 160, 188, 223, 73, 82], OperandSize::Dword)
}

#[test]
fn vpsllvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 71, 222], OperandSize::Qword)
}

#[test]
fn vpsllvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDI, 1752124288, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 71, 143, 128, 75, 111, 104], OperandSize::Qword)
}

#[test]
fn vpsllvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 143, 71, 234], OperandSize::Dword)
}

#[test]
fn vpsllvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 139, 71, 24], OperandSize::Dword)
}

#[test]
fn vpsllvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 155, 71, 20, 154], OperandSize::Dword)
}

#[test]
fn vpsllvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 197, 132, 71, 216], OperandSize::Qword)
}

#[test]
fn vpsllvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 229, 139, 71, 36, 146], OperandSize::Qword)
}

#[test]
fn vpsllvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 237, 148, 71, 51], OperandSize::Qword)
}

#[test]
fn vpsllvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 71, 227], OperandSize::Dword)
}

#[test]
fn vpsllvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1147062494, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 173, 71, 180, 119, 222, 200, 94, 68], OperandSize::Dword)
}

#[test]
fn vpsllvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 189, 71, 12, 73], OperandSize::Dword)
}

#[test]
fn vpsllvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 181, 173, 71, 197], OperandSize::Qword)
}

#[test]
fn vpsllvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM16)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 253, 163, 71, 19], OperandSize::Qword)
}

#[test]
fn vpsllvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 1984956588, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 165, 178, 71, 132, 200, 172, 8, 80, 118], OperandSize::Qword)
}

#[test]
fn vpsllvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 201, 71, 248], OperandSize::Dword)
}

#[test]
fn vpsllvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 203, 71, 41], OperandSize::Dword)
}

#[test]
fn vpsllvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1057923676, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 223, 71, 44, 197, 92, 162, 14, 63], OperandSize::Dword)
}

#[test]
fn vpsllvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 221, 196, 71, 231], OperandSize::Qword)
}

#[test]
fn vpsllvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 189, 207, 71, 0], OperandSize::Qword)
}

#[test]
fn vpsllvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSLLVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM14)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 141, 218, 71, 26], OperandSize::Qword)
}

