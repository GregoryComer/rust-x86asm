use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vptestmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 15, 39, 206], OperandSize::Dword)
}

fn vptestmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1936485783, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 13, 39, 188, 79, 151, 109, 108, 115], OperandSize::Dword)
}

fn vptestmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 26, 39, 28, 210], OperandSize::Dword)
}

fn vptestmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 10, 39, 211], OperandSize::Qword)
}

fn vptestmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RDX, 1894560300, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 7, 39, 162, 44, 178, 236, 112], OperandSize::Qword)
}

fn vptestmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 133, 28, 39, 52, 158], OperandSize::Qword)
}

fn vptestmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 41, 39, 232], OperandSize::Dword)
}

fn vptestmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 642279801, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 45, 39, 12, 69, 121, 105, 72, 38], OperandSize::Dword)
}

fn vptestmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 797698741, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 59, 39, 172, 243, 181, 234, 139, 47], OperandSize::Dword)
}

fn vptestmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 157, 36, 39, 240], OperandSize::Qword)
}

fn vptestmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1301962289, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 36, 39, 164, 195, 49, 94, 154, 77], OperandSize::Qword)
}

fn vptestmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RSI, 433639905, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 189, 51, 39, 150, 225, 209, 216, 25], OperandSize::Qword)
}

fn vptestmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 76, 39, 233], OperandSize::Dword)
}

fn vptestmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Four, 234241497, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 78, 39, 188, 136, 217, 61, 246, 13], OperandSize::Dword)
}

fn vptestmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 626489403, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 93, 39, 20, 181, 59, 120, 87, 37], OperandSize::Dword)
}

fn vptestmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 173, 65, 39, 212], OperandSize::Qword)
}

fn vptestmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 173, 69, 39, 47], OperandSize::Qword)
}

fn vptestmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPTESTMQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 87, 39, 43], OperandSize::Qword)
}

