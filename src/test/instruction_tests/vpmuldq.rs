use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 40, 255], OperandSize::Dword)
}

fn vpmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 40, 32], OperandSize::Dword)
}

fn vpmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 40, 200], OperandSize::Qword)
}

fn vpmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1560311218, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 40, 60, 245, 178, 117, 0, 93], OperandSize::Qword)
}

fn vpmuldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 40, 195], OperandSize::Dword)
}

fn vpmuldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Four, 1471499283, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 40, 148, 158, 19, 76, 181, 87], OperandSize::Dword)
}

fn vpmuldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 40, 206], OperandSize::Qword)
}

fn vpmuldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 530835825, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 40, 52, 189, 113, 233, 163, 31], OperandSize::Qword)
}

fn vpmuldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 141, 40, 241], OperandSize::Dword)
}

fn vpmuldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 556510327, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 138, 40, 180, 246, 119, 172, 43, 33], OperandSize::Dword)
}

fn vpmuldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 77769061, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 156, 40, 164, 138, 101, 169, 162, 4], OperandSize::Dword)
}

fn vpmuldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 141, 137, 40, 228], OperandSize::Qword)
}

fn vpmuldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 141, 139, 40, 20, 67], OperandSize::Qword)
}

fn vpmuldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 237, 158, 40, 60, 128], OperandSize::Qword)
}

fn vpmuldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 174, 40, 210], OperandSize::Dword)
}

fn vpmuldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 40, 12, 192], OperandSize::Dword)
}

fn vpmuldq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 187, 40, 12, 254], OperandSize::Dword)
}

fn vpmuldq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 181, 164, 40, 249], OperandSize::Qword)
}

fn vpmuldq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1388685173, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 173, 169, 40, 52, 221, 117, 167, 197, 82], OperandSize::Qword)
}

fn vpmuldq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 197, 181, 40, 28, 219], OperandSize::Qword)
}

fn vpmuldq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 203, 40, 215], OperandSize::Dword)
}

fn vpmuldq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 40, 8], OperandSize::Dword)
}

fn vpmuldq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EAX, ESI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 213, 223, 40, 12, 112], OperandSize::Dword)
}

fn vpmuldq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 157, 207, 40, 195], OperandSize::Qword)
}

fn vpmuldq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM13)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 149, 201, 40, 18], OperandSize::Qword)
}

fn vpmuldq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 173, 215, 40, 28, 155], OperandSize::Qword)
}

