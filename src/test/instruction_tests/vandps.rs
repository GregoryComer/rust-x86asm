use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 84, 192], OperandSize::Dword)
}

fn vandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 84, 12, 242], OperandSize::Dword)
}

fn vandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 84, 245], OperandSize::Qword)
}

fn vandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 84, 28, 179], OperandSize::Qword)
}

fn vandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 220, 84, 237], OperandSize::Dword)
}

fn vandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1461753155, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 84, 180, 158, 67, 149, 32, 87], OperandSize::Dword)
}

fn vandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 84, 193], OperandSize::Qword)
}

fn vandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 84, 54], OperandSize::Qword)
}

fn vandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 76, 139, 84, 218], OperandSize::Dword)
}

fn vandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EAX, 1233925211, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 84, 128, 91, 52, 140, 73], OperandSize::Dword)
}

fn vandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 1418290077, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 76, 154, 84, 52, 213, 157, 99, 137, 84], OperandSize::Dword)
}

fn vandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 92, 140, 84, 243], OperandSize::Qword)
}

fn vandps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 44, 129, 84, 20, 179], OperandSize::Qword)
}

fn vandps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 1228257250, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 52, 146, 84, 140, 182, 226, 183, 53, 73], OperandSize::Qword)
}

fn vandps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 92, 169, 84, 200], OperandSize::Dword)
}

fn vandps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 92, 174, 84, 43], OperandSize::Dword)
}

fn vandps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 189, 84, 41], OperandSize::Dword)
}

fn vandps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 76, 170, 84, 193], OperandSize::Qword)
}

fn vandps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 422371972, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 92, 162, 84, 140, 75, 132, 226, 44, 25], OperandSize::Qword)
}

fn vandps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RCX, 2135599048, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 181, 84, 153, 200, 167, 74, 127], OperandSize::Qword)
}

fn vandps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 108, 201, 84, 218], OperandSize::Dword)
}

fn vandps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDX, 610645387, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 100, 201, 84, 162, 139, 181, 101, 36], OperandSize::Dword)
}

fn vandps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 108, 221, 84, 38], OperandSize::Dword)
}

fn vandps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 193, 108, 199, 84, 255], OperandSize::Qword)
}

fn vandps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 113, 124, 197, 84, 27], OperandSize::Qword)
}

fn vandps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 225060591, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 213, 84, 188, 203, 239, 38, 106, 13], OperandSize::Qword)
}

