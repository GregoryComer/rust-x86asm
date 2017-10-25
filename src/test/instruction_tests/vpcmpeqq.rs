use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpeqq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 41, 247], OperandSize::Dword)
}

fn vpcmpeqq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 408304383, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 41, 36, 133, 255, 58, 86, 24], OperandSize::Dword)
}

fn vpcmpeqq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 41, 240], OperandSize::Qword)
}

fn vpcmpeqq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 762184040, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 41, 12, 213, 104, 1, 110, 45], OperandSize::Qword)
}

fn vpcmpeqq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 41, 253], OperandSize::Dword)
}

fn vpcmpeqq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 41, 36, 182], OperandSize::Dword)
}

fn vpcmpeqq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 41, 249], OperandSize::Qword)
}

fn vpcmpeqq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 806627467, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 41, 52, 93, 139, 40, 20, 48], OperandSize::Qword)
}

fn vpcmpeqq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 13, 41, 230], OperandSize::Dword)
}

fn vpcmpeqq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(ECX, 2107033587, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 13, 41, 185, 243, 199, 150, 125], OperandSize::Dword)
}

fn vpcmpeqq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Eight, 1203368218, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 229, 28, 41, 156, 254, 26, 241, 185, 71], OperandSize::Dword)
}

fn vpcmpeqq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 205, 7, 41, 246], OperandSize::Qword)
}

fn vpcmpeqq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 1312948679, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 173, 12, 41, 44, 93, 199, 1, 66, 78], OperandSize::Qword)
}

fn vpcmpeqq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 479199435, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 181, 20, 41, 180, 73, 203, 0, 144, 28], OperandSize::Qword)
}

fn vpcmpeqq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 43, 41, 238], OperandSize::Dword)
}

fn vpcmpeqq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 47, 41, 60, 139], OperandSize::Dword)
}

fn vpcmpeqq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 321609259, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 61, 41, 180, 127, 43, 94, 43, 19], OperandSize::Dword)
}

fn vpcmpeqq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 178, 133, 47, 41, 206], OperandSize::Qword)
}

fn vpcmpeqq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(RAX, 1807337745, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 44, 41, 176, 17, 201, 185, 107], OperandSize::Qword)
}

fn vpcmpeqq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 141, 61, 41, 60, 250], OperandSize::Qword)
}

fn vpcmpeqq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 75, 41, 232], OperandSize::Dword)
}

fn vpcmpeqq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 79, 41, 10], OperandSize::Dword)
}

fn vpcmpeqq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ESI, 874838385, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 94, 41, 182, 113, 249, 36, 52], OperandSize::Dword)
}

fn vpcmpeqq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 213, 79, 41, 201], OperandSize::Qword)
}

fn vpcmpeqq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 65, 41, 28, 66], OperandSize::Qword)
}

fn vpcmpeqq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQQ, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectDisplaced(RBX, 1692032348, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 86, 41, 139, 92, 93, 218, 100], OperandSize::Qword)
}

