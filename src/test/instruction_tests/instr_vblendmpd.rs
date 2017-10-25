use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 143, 101, 222], OperandSize::Dword)
}

#[test]
fn vblendmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 137, 101, 60, 115], OperandSize::Dword)
}

#[test]
fn vblendmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 153, 101, 44, 154], OperandSize::Dword)
}

#[test]
fn vblendmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 237, 135, 101, 249], OperandSize::Qword)
}

#[test]
fn vblendmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 646609194, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 141, 130, 101, 36, 69, 42, 121, 138, 38], OperandSize::Qword)
}

#[test]
fn vblendmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 302800249, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 181, 147, 101, 60, 181, 121, 93, 12, 18], OperandSize::Qword)
}

#[test]
fn vblendmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 101, 251], OperandSize::Dword)
}

#[test]
fn vblendmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 172, 101, 28, 211], OperandSize::Dword)
}

#[test]
fn vblendmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1932127557, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 190, 101, 164, 64, 69, 237, 41, 115], OperandSize::Dword)
}

#[test]
fn vblendmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 165, 161, 101, 231], OperandSize::Qword)
}

#[test]
fn vblendmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 830867558, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 173, 173, 101, 156, 152, 102, 8, 134, 49], OperandSize::Qword)
}

#[test]
fn vblendmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 199378446, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 205, 187, 101, 132, 131, 14, 70, 226, 11], OperandSize::Qword)
}

#[test]
fn vblendmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 206, 101, 203], OperandSize::Dword)
}

#[test]
fn vblendmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 206, 101, 62], OperandSize::Dword)
}

#[test]
fn vblendmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1330264678, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 218, 101, 172, 120, 102, 58, 74, 79], OperandSize::Dword)
}

#[test]
fn vblendmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 189, 194, 101, 204], OperandSize::Qword)
}

#[test]
fn vblendmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 181, 206, 101, 44, 207], OperandSize::Qword)
}

#[test]
fn vblendmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 563330246, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 173, 213, 101, 28, 189, 198, 188, 147, 33], OperandSize::Qword)
}

