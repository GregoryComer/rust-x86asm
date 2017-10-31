use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 171, 22, 242], OperandSize::Dword)
}

#[test]
fn vpermpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 173, 22, 44, 198], OperandSize::Dword)
}

#[test]
fn vpermpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 1941490668, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 187, 22, 145, 236, 203, 184, 115], OperandSize::Dword)
}

#[test]
fn vpermpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 213, 171, 22, 197], OperandSize::Qword)
}

#[test]
fn vpermpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 221, 167, 22, 2], OperandSize::Qword)
}

#[test]
fn vpermpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 610499221, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 181, 22, 188, 147, 149, 122, 99, 36], OperandSize::Qword)
}

#[test]
fn vpermpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 22, 213], OperandSize::Dword)
}

#[test]
fn vpermpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 816869457, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 22, 44, 221, 81, 112, 176, 48], OperandSize::Dword)
}

#[test]
fn vpermpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 222, 22, 44, 66], OperandSize::Dword)
}

#[test]
fn vpermpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 133, 197, 22, 214], OperandSize::Qword)
}

#[test]
fn vpermpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 253, 196, 22, 28, 83], OperandSize::Qword)
}

#[test]
fn vpermpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 245, 222, 22, 11], OperandSize::Qword)
}

#[test]
fn vpermpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 243, 108], OperandSize::Dword)
}

#[test]
fn vpermpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 911565528, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 140, 91, 216, 98, 85, 54, 24], OperandSize::Dword)
}

#[test]
fn vpermpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 235, 102], OperandSize::Qword)
}

#[test]
fn vpermpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 39, 22], OperandSize::Qword)
}

#[test]
fn vpermpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 1, 254, 37], OperandSize::Dword)
}

#[test]
fn vpermpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 456177546, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 253, 172, 1, 164, 112, 138, 183, 48, 27, 118], OperandSize::Dword)
}

#[test]
fn vpermpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 1293359082, Some(OperandSize::Qword), None)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 189, 1, 28, 189, 234, 23, 23, 77, 42], OperandSize::Dword)
}

#[test]
fn vpermpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM15)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 83, 253, 174, 1, 223, 27], OperandSize::Qword)
}

#[test]
fn vpermpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Four, 1865732830, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 170, 1, 172, 191, 222, 210, 52, 111, 5], OperandSize::Qword)
}

#[test]
fn vpermpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM16)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 366894130, Some(OperandSize::Qword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 227, 253, 186, 1, 4, 213, 50, 92, 222, 21, 26], OperandSize::Qword)
}

#[test]
fn vpermpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 253, 202, 1, 203, 72], OperandSize::Dword)
}

#[test]
fn vpermpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 267894179, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 203, 1, 28, 197, 163, 189, 247, 15, 35], OperandSize::Dword)
}

#[test]
fn vpermpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 222, 1, 52, 179, 67], OperandSize::Dword)
}

#[test]
fn vpermpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM10)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 211, 253, 203, 1, 218, 37], OperandSize::Qword)
}

#[test]
fn vpermpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(RBX, 736392313, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 253, 201, 1, 131, 121, 116, 228, 43, 110], OperandSize::Qword)
}

#[test]
fn vpermpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 219, 1, 4, 135, 85], OperandSize::Qword)
}

