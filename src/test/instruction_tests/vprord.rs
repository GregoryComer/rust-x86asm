use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vprord_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Literal8(93)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 77, 138, 114, 198, 93], OperandSize::Dword)
}

fn vprord_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 2131403762, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 109, 142, 114, 132, 249, 242, 163, 10, 127, 45], OperandSize::Dword)
}

fn vprord_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(EAX, 621889377, Some(OperandSize::Dword), None)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 69, 155, 114, 128, 97, 71, 17, 37, 57], OperandSize::Dword)
}

fn vprord_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM27)), operand3: Some(Literal8(108)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 13, 143, 114, 195, 108], OperandSize::Qword)
}

fn vprord_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 135, 114, 4, 127, 77], OperandSize::Qword)
}

fn vprord_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(XMM31)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 5, 150, 114, 6, 73], OperandSize::Qword)
}

fn vprord_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 77, 174, 114, 196, 30], OperandSize::Dword)
}

fn vprord_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 994997818, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 101, 170, 114, 132, 178, 58, 118, 78, 59, 29], OperandSize::Dword)
}

fn vprord_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectDisplaced(EDX, 1029875329, Some(OperandSize::Dword), None)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 69, 190, 114, 130, 129, 166, 98, 61, 53], OperandSize::Dword)
}

fn vprord_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM24)), operand3: Some(Literal8(34)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 29, 170, 114, 192, 34], OperandSize::Qword)
}

fn vprord_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 172, 114, 6, 85], OperandSize::Qword)
}

fn vprord_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(YMM17)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Eight, 391815153, Some(OperandSize::Dword), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 181, 114, 132, 249, 241, 159, 90, 23, 120], OperandSize::Qword)
}

fn vprord_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 117, 205, 114, 196, 43], OperandSize::Dword)
}

fn vprord_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(72)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 93, 202, 114, 1, 72], OperandSize::Dword)
}

fn vprord_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 85, 221, 114, 4, 203, 85], OperandSize::Dword)
}

fn vprord_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM26)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 145, 45, 207, 114, 194, 125], OperandSize::Qword)
}

fn vprord_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM13)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 21, 207, 114, 4, 215, 79], OperandSize::Qword)
}

fn vprord_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORD, operand1: Some(Direct(ZMM22)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 77, 215, 114, 1, 49], OperandSize::Qword)
}

