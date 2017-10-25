use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vprorvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 205, 137, 20, 244], OperandSize::Dword)
}

fn vprorvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 440835886, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 20, 170, 46, 159, 70, 26], OperandSize::Dword)
}

fn vprorvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 334074905, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 156, 20, 52, 141, 25, 148, 233, 19], OperandSize::Dword)
}

fn vprorvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 253, 138, 20, 209], OperandSize::Qword)
}

fn vprorvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 806105713, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 189, 140, 20, 188, 207, 113, 50, 12, 48], OperandSize::Qword)
}

fn vprorvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1386104502, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 149, 149, 20, 44, 117, 182, 70, 158, 82], OperandSize::Qword)
}

fn vprorvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 175, 20, 202], OperandSize::Dword)
}

fn vprorvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1063749919, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 170, 20, 159, 31, 137, 103, 63], OperandSize::Dword)
}

fn vprorvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 20, 3], OperandSize::Dword)
}

fn vprorvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 157, 172, 20, 201], OperandSize::Qword)
}

fn vprorvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 237, 165, 20, 20, 198], OperandSize::Qword)
}

fn vprorvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 141, 182, 20, 36, 242], OperandSize::Qword)
}

fn vprorvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 206, 20, 253], OperandSize::Dword)
}

fn vprorvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 203, 20, 35], OperandSize::Dword)
}

fn vprorvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 223, 20, 10], OperandSize::Dword)
}

fn vprorvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 229, 198, 20, 228], OperandSize::Qword)
}

fn vprorvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Four, 503597682, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 141, 199, 20, 180, 134, 114, 74, 4, 30], OperandSize::Qword)
}

fn vprorvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 708392739, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 141, 214, 20, 20, 93, 35, 55, 57, 42], OperandSize::Qword)
}

