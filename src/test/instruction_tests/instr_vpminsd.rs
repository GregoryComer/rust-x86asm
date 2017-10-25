use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 57, 192], OperandSize::Dword)
}

#[test]
fn vpminsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 57, 60, 195], OperandSize::Dword)
}

#[test]
fn vpminsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 57, 210], OperandSize::Qword)
}

#[test]
fn vpminsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 57, 43], OperandSize::Qword)
}

#[test]
fn vpminsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 57, 232], OperandSize::Dword)
}

#[test]
fn vpminsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 57, 7], OperandSize::Dword)
}

#[test]
fn vpminsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 57, 197], OperandSize::Qword)
}

#[test]
fn vpminsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 453610108, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 57, 44, 141, 124, 138, 9, 27], OperandSize::Qword)
}

#[test]
fn vpminsd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 138, 57, 208], OperandSize::Dword)
}

#[test]
fn vpminsd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ECX, 388648793, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 57, 177, 89, 79, 42, 23], OperandSize::Dword)
}

#[test]
fn vpminsd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 2045657952, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 101, 159, 57, 4, 149, 96, 67, 238, 121], OperandSize::Dword)
}

#[test]
fn vpminsd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 5, 141, 57, 204], OperandSize::Qword)
}

#[test]
fn vpminsd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 853121071, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 21, 133, 57, 52, 93, 47, 152, 217, 50], OperandSize::Qword)
}

#[test]
fn vpminsd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 21, 154, 57, 28, 72], OperandSize::Qword)
}

#[test]
fn vpminsd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 175, 57, 219], OperandSize::Dword)
}

#[test]
fn vpminsd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 170, 57, 19], OperandSize::Dword)
}

#[test]
fn vpminsd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 187, 57, 23], OperandSize::Dword)
}

#[test]
fn vpminsd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 101, 170, 57, 198], OperandSize::Qword)
}

#[test]
fn vpminsd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectDisplaced(RCX, 1983309477, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 77, 167, 57, 145, 165, 230, 54, 118], OperandSize::Qword)
}

#[test]
fn vpminsd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 187, 57, 33], OperandSize::Qword)
}

#[test]
fn vpminsd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 206, 57, 241], OperandSize::Dword)
}

#[test]
fn vpminsd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 57, 24], OperandSize::Dword)
}

#[test]
fn vpminsd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ECX, 1028534585, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 219, 57, 161, 57, 49, 78, 61], OperandSize::Dword)
}

#[test]
fn vpminsd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 53, 203, 57, 205], OperandSize::Qword)
}

#[test]
fn vpminsd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(RSI, 972029515, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 85, 205, 57, 142, 75, 254, 239, 57], OperandSize::Qword)
}

#[test]
fn vpminsd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 45, 221, 57, 20, 146], OperandSize::Qword)
}

