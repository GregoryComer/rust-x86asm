use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 237, 15, 31, 241, 3], OperandSize::Dword)
}

#[test]
fn vpcmpq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 1365730470, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 253, 13, 31, 148, 145, 166, 100, 103, 81, 70], OperandSize::Dword)
}

#[test]
fn vpcmpq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 1348066465, Some(OperandSize::Qword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 205, 29, 31, 188, 249, 161, 220, 89, 80, 125], OperandSize::Dword)
}

#[test]
fn vpcmpq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM9)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 211, 157, 5, 31, 249, 104], OperandSize::Qword)
}

#[test]
fn vpcmpq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 814336313, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 221, 14, 31, 20, 77, 57, 201, 137, 48, 18], OperandSize::Qword)
}

#[test]
fn vpcmpq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 579829768, Some(OperandSize::Qword), None)), operand4: Some(Literal8(8)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 149, 29, 31, 180, 123, 8, 128, 143, 34, 8], OperandSize::Qword)
}

#[test]
fn vpcmpq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 237, 41, 31, 242, 40], OperandSize::Dword)
}

#[test]
fn vpcmpq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Two, 834159782, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 43, 31, 180, 75, 166, 68, 184, 49, 54], OperandSize::Dword)
}

#[test]
fn vpcmpq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Qword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 213, 57, 31, 20, 65, 98], OperandSize::Dword)
}

#[test]
fn vpcmpq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM24)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 147, 245, 33, 31, 240, 124], OperandSize::Qword)
}

#[test]
fn vpcmpq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1307660467, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(94)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 213, 42, 31, 36, 205, 179, 80, 241, 77, 94], OperandSize::Qword)
}

#[test]
fn vpcmpq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1101749743, Some(OperandSize::Qword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 205, 54, 31, 36, 117, 239, 93, 171, 65, 124], OperandSize::Qword)
}

#[test]
fn vpcmpq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 245, 73, 31, 209, 13], OperandSize::Dword)
}

#[test]
fn vpcmpq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 603741712, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 197, 78, 31, 188, 83, 16, 94, 252, 35, 75], OperandSize::Dword)
}

#[test]
fn vpcmpq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1977072887, Some(OperandSize::Qword), None)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 205, 90, 31, 172, 182, 247, 188, 215, 117, 58], OperandSize::Dword)
}

#[test]
fn vpcmpq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM25)), operand3: Some(Direct(ZMM24)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 147, 181, 70, 31, 224, 86], OperandSize::Qword)
}

#[test]
fn vpcmpq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1844938209, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 189, 79, 31, 156, 158, 225, 133, 247, 109, 60], OperandSize::Qword)
}

#[test]
fn vpcmpq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectDisplaced(RSI, 927134282, Some(OperandSize::Qword), None)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 173, 85, 31, 150, 74, 242, 66, 55, 5], OperandSize::Qword)
}

