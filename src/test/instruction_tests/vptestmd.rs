use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptestmd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 12, 39, 229], OperandSize::Dword)
}

fn vptestmd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 87403684, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 12, 39, 188, 119, 164, 172, 53, 5], OperandSize::Dword)
}

fn vptestmd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Four, 1346190595, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 31, 39, 156, 159, 3, 61, 61, 80], OperandSize::Dword)
}

fn vptestmd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 77, 4, 39, 254], OperandSize::Qword)
}

fn vptestmd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Two, 176876362, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 93, 5, 39, 172, 72, 74, 235, 138, 10], OperandSize::Qword)
}

fn vptestmd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 21, 31, 39, 52, 183], OperandSize::Qword)
}

fn vptestmd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 46, 39, 242], OperandSize::Dword)
}

fn vptestmd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 1430901594, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 42, 39, 167, 90, 211, 73, 85], OperandSize::Dword)
}

fn vptestmd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 811820010, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 93, 60, 39, 20, 157, 234, 99, 99, 48], OperandSize::Dword)
}

fn vptestmd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 69, 45, 39, 247], OperandSize::Qword)
}

fn vptestmd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RDX, 1787446119, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 53, 38, 39, 186, 103, 67, 138, 106], OperandSize::Qword)
}

fn vptestmd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM13)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1655587830, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 21, 60, 39, 28, 253, 246, 67, 174, 98], OperandSize::Qword)
}

fn vptestmd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 75, 39, 255], OperandSize::Dword)
}

fn vptestmd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1756669207, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 77, 79, 39, 12, 133, 23, 165, 180, 104], OperandSize::Dword)
}

fn vptestmd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 69, 89, 39, 34], OperandSize::Dword)
}

fn vptestmd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM31)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 5, 70, 39, 202], OperandSize::Qword)
}

fn vptestmd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 65, 39, 51], OperandSize::Qword)
}

fn vptestmd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Eight, 1197963438, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 93, 39, 164, 251, 174, 120, 103, 71], OperandSize::Qword)
}

