use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 85, 143, 21, 222], OperandSize::Dword)
}

#[test]
fn vprolvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 85, 138, 21, 12, 247], OperandSize::Dword)
}

#[test]
fn vprolvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1825231794, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 156, 21, 28, 189, 178, 211, 202, 108], OperandSize::Dword)
}

#[test]
fn vprolvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 61, 132, 21, 196], OperandSize::Qword)
}

#[test]
fn vprolvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 117, 141, 21, 20, 122], OperandSize::Qword)
}

#[test]
fn vprolvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 86950083, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 153, 21, 140, 119, 195, 192, 46, 5], OperandSize::Qword)
}

#[test]
fn vprolvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 171, 21, 199], OperandSize::Dword)
}

#[test]
fn vprolvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1544195484, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 93, 175, 21, 183, 156, 141, 10, 92], OperandSize::Dword)
}

#[test]
fn vprolvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Eight, 1816971655, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 85, 190, 21, 132, 192, 135, 201, 76, 108], OperandSize::Dword)
}

#[test]
fn vprolvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 21, 165, 21, 222], OperandSize::Qword)
}

#[test]
fn vprolvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectDisplaced(RSI, 106277400, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 37, 174, 21, 174, 24, 170, 85, 6], OperandSize::Qword)
}

#[test]
fn vprolvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Two, 264400533, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 37, 183, 21, 132, 64, 149, 110, 194, 15], OperandSize::Qword)
}

#[test]
fn vprolvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 203, 21, 237], OperandSize::Dword)
}

#[test]
fn vprolvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EDX, 1556200787, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 21, 162, 83, 189, 193, 92], OperandSize::Dword)
}

#[test]
fn vprolvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 1805491787, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 217, 21, 156, 112, 75, 158, 157, 107], OperandSize::Dword)
}

#[test]
fn vprolvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 178, 117, 194, 21, 238], OperandSize::Qword)
}

#[test]
fn vprolvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM11)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 37, 202, 21, 39], OperandSize::Qword)
}

#[test]
fn vprolvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 322117834, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 77, 219, 21, 180, 203, 202, 32, 51, 19], OperandSize::Qword)
}

