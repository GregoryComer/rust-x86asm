use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 142, 114, 194, 78], OperandSize::Dword)
}

#[test]
fn vprorq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EBX, 522967832, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 213, 140, 114, 131, 24, 219, 43, 31, 65], OperandSize::Dword)
}

#[test]
fn vprorq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 197, 155, 114, 4, 254, 96], OperandSize::Dword)
}

#[test]
fn vprorq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM23)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 177, 253, 129, 114, 199, 45], OperandSize::Qword)
}

#[test]
fn vprorq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM11)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 165, 142, 114, 6, 100], OperandSize::Qword)
}

#[test]
fn vprorq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 654934440, Some(OperandSize::Qword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 150, 114, 4, 157, 168, 129, 9, 39, 126], OperandSize::Qword)
}

#[test]
fn vprorq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 213, 173, 114, 198, 18], OperandSize::Dword)
}

#[test]
fn vprorq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 245, 175, 114, 4, 127, 34], OperandSize::Dword)
}

#[test]
fn vprorq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 229, 185, 114, 2, 23], OperandSize::Dword)
}

#[test]
fn vprorq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(118)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 197, 162, 114, 197, 118], OperandSize::Qword)
}

#[test]
fn vprorq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 74745794, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 167, 114, 132, 136, 194, 135, 116, 4, 127], OperandSize::Qword)
}

#[test]
fn vprorq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(YMM26)), operand2: Some(IndirectDisplaced(RDI, 2120084305, Some(OperandSize::Qword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 173, 182, 114, 135, 81, 235, 93, 126, 83], OperandSize::Qword)
}

#[test]
fn vprorq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 237, 204, 114, 198, 22], OperandSize::Dword)
}

#[test]
fn vprorq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 114, 7, 76], OperandSize::Dword)
}

#[test]
fn vprorq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 221, 114, 0, 36], OperandSize::Dword)
}

#[test]
fn vprorq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 221, 195, 114, 193, 68], OperandSize::Qword)
}

#[test]
fn vprorq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM21)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 213, 198, 114, 7, 113], OperandSize::Qword)
}

#[test]
fn vprorq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORQ, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 211, 114, 4, 65, 90], OperandSize::Qword)
}

