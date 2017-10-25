use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 41, 192], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1155022041, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 41, 132, 79, 217, 60, 216, 68], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 41, 205], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 41, 4, 71], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 41, 240], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 41, 36, 70], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 41, 225], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 1803673673, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 41, 138, 73, 224, 129, 107], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 14, 41, 226], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 181862961, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 12, 41, 153, 49, 2, 215, 10], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 25, 41, 44, 115], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 178, 189, 9, 41, 234], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 11, 41, 24], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 168202578, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 29, 41, 52, 141, 82, 145, 6, 10], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 46, 41, 214], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 44, 41, 11], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 834274046, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 62, 41, 160, 254, 2, 186, 49], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 43, 41, 210], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 141, 47, 41, 47], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM23)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 55, 41, 30], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 73, 41, 248], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(ECX, 1148594044, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 74, 41, 185, 124, 39, 118, 68], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 90, 41, 12, 190], OperandSize::Dword)
}

#[test]
fn vpcmpeqq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM11)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 165, 74, 41, 208], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 296422725, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 181, 71, 41, 164, 75, 69, 13, 171, 17], OperandSize::Qword)
}

#[test]
fn vpcmpeqq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 141, 83, 41, 60, 207], OperandSize::Qword)
}

