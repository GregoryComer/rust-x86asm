use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 40, 247], OperandSize::Dword)
}

#[test]
fn vpmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 40, 3], OperandSize::Dword)
}

#[test]
fn vpmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 40, 203], OperandSize::Qword)
}

#[test]
fn vpmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 40, 20, 71], OperandSize::Qword)
}

#[test]
fn vpmuldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 40, 206], OperandSize::Dword)
}

#[test]
fn vpmuldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(EDX, 729638066, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 40, 178, 178, 100, 125, 43], OperandSize::Dword)
}

#[test]
fn vpmuldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 40, 220], OperandSize::Qword)
}

#[test]
fn vpmuldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 40, 20, 201], OperandSize::Qword)
}

#[test]
fn vpmuldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 245, 140, 40, 208], OperandSize::Dword)
}

#[test]
fn vpmuldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 1861667188, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 40, 36, 77, 116, 201, 246, 110], OperandSize::Dword)
}

#[test]
fn vpmuldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 509837703, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 40, 12, 253, 135, 129, 99, 30], OperandSize::Dword)
}

#[test]
fn vpmuldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 165, 132, 40, 198], OperandSize::Qword)
}

#[test]
fn vpmuldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1742493640, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 221, 134, 40, 188, 215, 200, 87, 220, 103], OperandSize::Qword)
}

#[test]
fn vpmuldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 778030617, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 156, 40, 20, 93, 25, 206, 95, 46], OperandSize::Qword)
}

#[test]
fn vpmuldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 40, 214], OperandSize::Dword)
}

#[test]
fn vpmuldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(ESI, 1563689643, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 173, 40, 150, 171, 2, 52, 93], OperandSize::Dword)
}

#[test]
fn vpmuldq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ESI, 1379472430, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 245, 186, 40, 134, 46, 20, 57, 82], OperandSize::Dword)
}

#[test]
fn vpmuldq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 245, 167, 40, 255], OperandSize::Qword)
}

#[test]
fn vpmuldq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 141, 175, 40, 4, 199], OperandSize::Qword)
}

#[test]
fn vpmuldq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM25)), operand3: Some(IndirectDisplaced(RSI, 802863241, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 181, 178, 40, 142, 137, 184, 218, 47], OperandSize::Qword)
}

#[test]
fn vpmuldq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 205, 205, 40, 213], OperandSize::Dword)
}

#[test]
fn vpmuldq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EAX, 114600073, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 40, 144, 137, 168, 212, 6], OperandSize::Dword)
}

#[test]
fn vpmuldq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ESI, 769526484, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 218, 40, 182, 212, 10, 222, 45], OperandSize::Dword)
}

#[test]
fn vpmuldq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 149, 204, 40, 233], OperandSize::Qword)
}

#[test]
fn vpmuldq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1102318502, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 229, 196, 40, 52, 149, 166, 11, 180, 65], OperandSize::Qword)
}

#[test]
fn vpmuldq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 1017036121, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 181, 217, 40, 172, 208, 89, 189, 158, 60], OperandSize::Qword)
}

