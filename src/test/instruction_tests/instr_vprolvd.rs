use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 139, 21, 200], OperandSize::Dword)
}

#[test]
fn vprolvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 21, 60, 241], OperandSize::Dword)
}

#[test]
fn vprolvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1683089952, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 154, 21, 164, 178, 32, 234, 81, 100], OperandSize::Dword)
}

#[test]
fn vprolvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 117, 135, 21, 224], OperandSize::Qword)
}

#[test]
fn vprolvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1367467603, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 101, 143, 21, 148, 144, 83, 230, 129, 81], OperandSize::Qword)
}

#[test]
fn vprolvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 187899353, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 77, 153, 21, 4, 181, 217, 29, 51, 11], OperandSize::Qword)
}

#[test]
fn vprolvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 21, 231], OperandSize::Dword)
}

#[test]
fn vprolvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1226299852, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 172, 21, 28, 93, 204, 217, 23, 73], OperandSize::Dword)
}

#[test]
fn vprolvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 790001940, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 69, 189, 21, 44, 181, 20, 121, 22, 47], OperandSize::Dword)
}

#[test]
fn vprolvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 13, 164, 21, 218], OperandSize::Qword)
}

#[test]
fn vprolvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM13)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 21, 175, 21, 34], OperandSize::Qword)
}

#[test]
fn vprolvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM30)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 13, 181, 21, 49], OperandSize::Qword)
}

#[test]
fn vprolvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 207, 21, 247], OperandSize::Dword)
}

#[test]
fn vprolvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 2019319888, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 85, 205, 21, 152, 80, 96, 92, 120], OperandSize::Dword)
}

#[test]
fn vprolvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 1017146113, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 85, 221, 21, 163, 1, 107, 160, 60], OperandSize::Dword)
}

#[test]
fn vprolvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 13, 204, 21, 230], OperandSize::Qword)
}

#[test]
fn vprolvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectDisplaced(RDI, 2077796971, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 5, 205, 21, 175, 107, 170, 216, 123], OperandSize::Qword)
}

#[test]
fn vprolvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1139260670, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 53, 221, 21, 44, 133, 254, 188, 231, 67], OperandSize::Qword)
}

