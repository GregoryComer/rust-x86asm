use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 221, 11, 31, 228, 6], OperandSize::Dword)
}

fn vpcmpq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 677964580, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(16)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 237, 12, 31, 164, 255, 36, 235, 104, 40, 16], OperandSize::Dword)
}

fn vpcmpq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1394071952, Some(OperandSize::Qword), None)), operand4: Some(Literal8(86)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 221, 27, 31, 188, 90, 144, 217, 23, 83, 86], OperandSize::Dword)
}

fn vpcmpq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 221, 6, 31, 238, 47], OperandSize::Qword)
}

fn vpcmpq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 157, 13, 31, 60, 64, 104], OperandSize::Qword)
}

fn vpcmpq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 243, 149, 30, 31, 44, 207, 18], OperandSize::Qword)
}

fn vpcmpq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 46, 31, 203, 11], OperandSize::Dword)
}

fn vpcmpq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(33)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 213, 46, 31, 23, 33], OperandSize::Dword)
}

fn vpcmpq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1291794352, Some(OperandSize::Qword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 245, 61, 31, 20, 157, 176, 55, 255, 76, 62], OperandSize::Dword)
}

fn vpcmpq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM17)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 173, 42, 31, 249, 30], OperandSize::Qword)
}

fn vpcmpq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 1284878151, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 173, 39, 31, 156, 198, 71, 175, 149, 76, 13], OperandSize::Qword)
}

fn vpcmpq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 213, 60, 31, 60, 216, 112], OperandSize::Qword)
}

fn vpcmpq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 237, 75, 31, 254, 42], OperandSize::Dword)
}

fn vpcmpq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 75, 31, 60, 178, 11], OperandSize::Dword)
}

fn vpcmpq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 229, 90, 31, 49, 23], OperandSize::Dword)
}

fn vpcmpq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM17)), operand4: Some(Literal8(3)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 179, 213, 69, 31, 225, 3], OperandSize::Qword)
}

fn vpcmpq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(98)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 189, 75, 31, 28, 251, 98], OperandSize::Qword)
}

fn vpcmpq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 1123974136, Some(OperandSize::Qword), None)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 149, 81, 31, 188, 158, 248, 123, 254, 66, 17], OperandSize::Qword)
}

