use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermi2d_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 139, 118, 250], OperandSize::Dword)
}

fn vpermi2d_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(EDI, 1030477968, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 101, 138, 118, 175, 144, 216, 107, 61], OperandSize::Dword)
}

fn vpermi2d_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 606302953, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 117, 159, 118, 180, 199, 233, 114, 35, 36], OperandSize::Dword)
}

fn vpermi2d_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 117, 141, 118, 197], OperandSize::Qword)
}

fn vpermi2d_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 636206370, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 53, 137, 118, 12, 149, 34, 189, 235, 37], OperandSize::Qword)
}

fn vpermi2d_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 77, 155, 118, 42], OperandSize::Qword)
}

fn vpermi2d_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 174, 118, 207], OperandSize::Dword)
}

fn vpermi2d_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 5728790, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 118, 162, 22, 106, 87, 0], OperandSize::Dword)
}

fn vpermi2d_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1649946865, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 186, 118, 132, 144, 241, 48, 88, 98], OperandSize::Dword)
}

fn vpermi2d_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM14)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 13, 174, 118, 249], OperandSize::Qword)
}

fn vpermi2d_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 1782953582, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 117, 172, 118, 156, 119, 110, 182, 69, 106], OperandSize::Qword)
}

fn vpermi2d_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 187, 118, 50], OperandSize::Qword)
}

fn vpermi2d_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 206, 118, 212], OperandSize::Dword)
}

fn vpermi2d_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1430944657, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 207, 118, 156, 144, 145, 123, 74, 85], OperandSize::Dword)
}

fn vpermi2d_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 222, 118, 54], OperandSize::Dword)
}

fn vpermi2d_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 109, 202, 118, 249], OperandSize::Qword)
}

fn vpermi2d_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RSI, 964041418, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 109, 204, 118, 158, 202, 26, 118, 57], OperandSize::Qword)
}

fn vpermi2d_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2D, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 37, 218, 118, 12, 206], OperandSize::Qword)
}

