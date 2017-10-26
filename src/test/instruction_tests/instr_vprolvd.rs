use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 21, 225], OperandSize::Dword)
}

#[test]
fn vprolvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 21, 30], OperandSize::Dword)
}

#[test]
fn vprolvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 652301467, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 69, 153, 21, 164, 219, 155, 84, 225, 38], OperandSize::Dword)
}

#[test]
fn vprolvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 125, 130, 21, 243], OperandSize::Qword)
}

#[test]
fn vprolvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 13, 139, 21, 1], OperandSize::Qword)
}

#[test]
fn vprolvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 117, 155, 21, 36, 246], OperandSize::Qword)
}

#[test]
fn vprolvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 21, 239], OperandSize::Dword)
}

#[test]
fn vprolvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 682075309, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 21, 188, 178, 173, 164, 167, 40], OperandSize::Dword)
}

#[test]
fn vprolvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 552707820, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 190, 21, 12, 117, 236, 166, 241, 32], OperandSize::Dword)
}

#[test]
fn vprolvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 77, 171, 21, 215], OperandSize::Qword)
}

#[test]
fn vprolvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM18)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 166, 21, 24], OperandSize::Qword)
}

#[test]
fn vprolvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1210722553, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 53, 180, 21, 12, 93, 249, 40, 42, 72], OperandSize::Qword)
}

#[test]
fn vprolvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 204, 21, 195], OperandSize::Dword)
}

#[test]
fn vprolvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 205, 21, 36, 130], OperandSize::Dword)
}

#[test]
fn vprolvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1517609797, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 219, 21, 164, 243, 69, 227, 116, 90], OperandSize::Dword)
}

#[test]
fn vprolvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 109, 199, 21, 210], OperandSize::Qword)
}

#[test]
fn vprolvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1835939967, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 21, 148, 186, 127, 56, 110, 109], OperandSize::Qword)
}

#[test]
fn vprolvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1321864081, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 109, 214, 21, 44, 181, 145, 11, 202, 78], OperandSize::Qword)
}

