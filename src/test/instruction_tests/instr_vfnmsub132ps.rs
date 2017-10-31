use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub132ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 158, 229], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Eight, 1649453611, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 158, 140, 207, 43, 170, 80, 98], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 158, 240], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 1357057315, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 158, 162, 35, 13, 227, 80], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 158, 244], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 158, 57], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 158, 235], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Four, 1973594595, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 158, 156, 179, 227, 169, 162, 117], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 69, 139, 158, 215], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 158, 1], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Four, 60109427, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 154, 158, 172, 147, 115, 50, 149, 3], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 117, 141, 158, 214], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM9)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 53, 138, 158, 39], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 117, 155, 158, 19], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 77, 171, 158, 228], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 715025660, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 158, 140, 210, 252, 108, 158, 42], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 190, 158, 23], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 77, 166, 158, 248], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 1980177911, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 77, 166, 158, 132, 248, 247, 29, 7, 118], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 61, 189, 158, 36, 151], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 153, 158, 200], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 203, 158, 9], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(ECX, 1539625596, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 222, 158, 153, 124, 210, 196, 91], OperandSize::Dword)
}

#[test]
fn vfnmsub132ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM29)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 109, 181, 158, 197], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1569846667, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 85, 197, 158, 4, 125, 139, 245, 145, 93], OperandSize::Qword)
}

#[test]
fn vfnmsub132ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB132PS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1454666902, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 117, 219, 158, 28, 205, 150, 116, 180, 86], OperandSize::Qword)
}

