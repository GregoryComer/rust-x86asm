use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmuludq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 244, 205], OperandSize::Dword)
}

#[test]
fn vpmuludq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 244, 20, 112], OperandSize::Dword)
}

#[test]
fn vpmuludq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 244, 241], OperandSize::Qword)
}

#[test]
fn vpmuludq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 244, 28, 66], OperandSize::Qword)
}

#[test]
fn vpmuludq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 244, 210], OperandSize::Dword)
}

#[test]
fn vpmuludq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 244, 49], OperandSize::Dword)
}

#[test]
fn vpmuludq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 244, 199], OperandSize::Qword)
}

#[test]
fn vpmuludq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RSI, 1741831055, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 244, 134, 143, 59, 210, 103], OperandSize::Qword)
}

#[test]
fn vpmuludq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 237, 138, 244, 244], OperandSize::Dword)
}

#[test]
fn vpmuludq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 773076688, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 138, 244, 180, 114, 208, 54, 20, 46], OperandSize::Dword)
}

#[test]
fn vpmuludq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 153, 244, 50], OperandSize::Dword)
}

#[test]
fn vpmuludq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 193, 245, 141, 244, 225], OperandSize::Qword)
}

#[test]
fn vpmuludq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 229, 134, 244, 62], OperandSize::Qword)
}

#[test]
fn vpmuludq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 155, 244, 42], OperandSize::Qword)
}

#[test]
fn vpmuludq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 205, 170, 244, 225], OperandSize::Dword)
}

#[test]
fn vpmuludq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 199273423, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 213, 170, 244, 170, 207, 171, 224, 11], OperandSize::Dword)
}

#[test]
fn vpmuludq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 245, 186, 244, 4, 158], OperandSize::Dword)
}

#[test]
fn vpmuludq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 157, 172, 244, 221], OperandSize::Qword)
}

#[test]
fn vpmuludq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectDisplaced(RDI, 1003911320, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 141, 167, 244, 191, 152, 120, 214, 59], OperandSize::Qword)
}

#[test]
fn vpmuludq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 869979893, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 237, 188, 244, 52, 85, 245, 214, 218, 51], OperandSize::Qword)
}

#[test]
fn vpmuludq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 197, 202, 244, 241], OperandSize::Dword)
}

#[test]
fn vpmuludq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 755299680, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 204, 244, 132, 90, 96, 245, 4, 45], OperandSize::Dword)
}

#[test]
fn vpmuludq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(ESI, 1774625310, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 221, 222, 244, 174, 30, 162, 198, 105], OperandSize::Dword)
}

#[test]
fn vpmuludq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 141, 203, 244, 240], OperandSize::Qword)
}

#[test]
fn vpmuludq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 237, 207, 244, 28, 251], OperandSize::Qword)
}

#[test]
fn vpmuludq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULUDQ, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 197, 215, 244, 52, 70], OperandSize::Qword)
}

