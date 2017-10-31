use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub213ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 166, 234], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Eight, 261552592, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 166, 180, 195, 208, 249, 150, 15], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 166, 235], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 166, 14], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 166, 215], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 166, 54], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 166, 240], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 166, 58], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 139, 166, 233], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1263073197, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 166, 148, 219, 173, 247, 72, 75], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 153, 166, 52, 89], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 61, 133, 166, 206], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 45, 133, 166, 15], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 1491093625, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 37, 145, 166, 28, 141, 121, 72, 224, 88], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 166, 251], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 518212932, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 173, 166, 164, 122, 68, 77, 227, 30], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 691601216, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 189, 166, 182, 64, 255, 56, 41], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 169, 166, 247], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 93, 172, 166, 44, 152], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 187, 166, 54], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 251, 166, 232], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1892539191, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 203, 166, 164, 200, 55, 219, 205, 112], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1008671334, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 218, 166, 140, 218, 102, 26, 31, 60], OperandSize::Dword)
}

#[test]
fn vfmaddsub213ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM29)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 21, 241, 166, 210], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RDX, 2130448723, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 109, 197, 166, 130, 83, 17, 252, 126], OperandSize::Qword)
}

#[test]
fn vfmaddsub213ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 53, 214, 166, 28, 147], OperandSize::Qword)
}

