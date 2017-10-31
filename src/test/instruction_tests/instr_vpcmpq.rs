use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 213, 15, 31, 202, 122], OperandSize::Dword)
}

#[test]
fn vpcmpq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 205, 11, 31, 58, 11], OperandSize::Dword)
}

#[test]
fn vpcmpq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 2043205876, Some(OperandSize::Qword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 197, 30, 31, 180, 254, 244, 216, 200, 121, 69], OperandSize::Dword)
}

#[test]
fn vpcmpq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 173, 10, 31, 254, 37], OperandSize::Qword)
}

#[test]
fn vpcmpq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 532315644, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 221, 9, 31, 172, 186, 252, 125, 186, 31, 60], OperandSize::Qword)
}

#[test]
fn vpcmpq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 1446011186, Some(OperandSize::Qword), None)), operand4: Some(Literal8(111)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 197, 26, 31, 184, 50, 97, 48, 86, 111], OperandSize::Qword)
}

#[test]
fn vpcmpq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 229, 42, 31, 204, 56], OperandSize::Dword)
}

#[test]
fn vpcmpq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 45, 31, 20, 147, 17], OperandSize::Dword)
}

#[test]
fn vpcmpq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 237, 59, 31, 20, 94, 79], OperandSize::Dword)
}

#[test]
fn vpcmpq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM16)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 173, 45, 31, 248, 122], OperandSize::Qword)
}

#[test]
fn vpcmpq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1070068578, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(83)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 181, 43, 31, 52, 197, 98, 243, 199, 63, 83], OperandSize::Qword)
}

#[test]
fn vpcmpq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1163702833, Some(OperandSize::Qword), None)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 133, 62, 31, 52, 149, 49, 178, 92, 69, 63], OperandSize::Qword)
}

#[test]
fn vpcmpq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(20)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 229, 79, 31, 240, 20], OperandSize::Dword)
}

#[test]
fn vpcmpq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1177840850, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(24)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 245, 75, 31, 148, 91, 210, 108, 52, 70, 24], OperandSize::Dword)
}

#[test]
fn vpcmpq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 213, 90, 31, 8, 72], OperandSize::Dword)
}

#[test]
fn vpcmpq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM23)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 221, 65, 31, 231, 121], OperandSize::Qword)
}

#[test]
fn vpcmpq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 245, 69, 31, 42, 96], OperandSize::Qword)
}

#[test]
fn vpcmpq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 149, 85, 31, 51, 89], OperandSize::Qword)
}

