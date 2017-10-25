use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfpclassps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 12, 102, 254, 93], OperandSize::Dword)
}

#[test]
fn vfpclassps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(IndirectDisplaced(EDX, 389898884, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 9, 102, 146, 132, 98, 61, 23, 104], OperandSize::Dword)
}

#[test]
fn vfpclassps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 29, 102, 33, 108], OperandSize::Dword)
}

#[test]
fn vfpclassps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM8)), operand3: Some(Literal8(75)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 125, 10, 102, 208, 75], OperandSize::Qword)
}

#[test]
fn vfpclassps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 10, 102, 12, 217, 126], OperandSize::Qword)
}

#[test]
fn vfpclassps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 125, 28, 102, 28, 81, 70], OperandSize::Qword)
}

#[test]
fn vfpclassps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(74)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 44, 102, 212, 74], OperandSize::Dword)
}

#[test]
fn vfpclassps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 2044952493, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(47)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 125, 45, 102, 44, 181, 173, 127, 227, 121, 47], OperandSize::Dword)
}

#[test]
fn vfpclassps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectDisplaced(ESI, 421885354, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 62, 102, 142, 170, 117, 37, 25, 53], OperandSize::Dword)
}

#[test]
fn vfpclassps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM12)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 211, 125, 45, 102, 204, 83], OperandSize::Qword)
}

#[test]
fn vfpclassps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K2)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 125, 46, 102, 19, 45], OperandSize::Qword)
}

#[test]
fn vfpclassps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K4)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 571886925, Some(OperandSize::Dword), None)), operand3: Some(Literal8(105)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 125, 63, 102, 36, 125, 77, 77, 22, 34, 105], OperandSize::Qword)
}

#[test]
fn vfpclassps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(116)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 74, 102, 249, 116], OperandSize::Dword)
}

#[test]
fn vfpclassps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K6)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 73, 102, 52, 243, 92], OperandSize::Dword)
}

#[test]
fn vfpclassps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Dword), None)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 91, 102, 12, 86, 19], OperandSize::Dword)
}

#[test]
fn vfpclassps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM17)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 179, 125, 76, 102, 233, 27], OperandSize::Qword)
}

#[test]
fn vfpclassps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1109804184, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 73, 102, 156, 207, 152, 68, 38, 66, 49], OperandSize::Qword)
}

#[test]
fn vfpclassps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFPCLASSPS, operand1: Some(Direct(K1)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Dword), None)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 125, 91, 102, 12, 150, 100], OperandSize::Qword)
}

