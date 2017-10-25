use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 40, 253], OperandSize::Dword)
}

#[test]
fn vpmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 40, 49], OperandSize::Dword)
}

#[test]
fn vpmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 40, 204], OperandSize::Qword)
}

#[test]
fn vpmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1349436548, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 40, 60, 181, 132, 196, 110, 80], OperandSize::Qword)
}

#[test]
fn vpmuldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 40, 231], OperandSize::Dword)
}

#[test]
fn vpmuldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1330737691, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 40, 164, 176, 27, 114, 81, 79], OperandSize::Dword)
}

#[test]
fn vpmuldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 40, 221], OperandSize::Qword)
}

#[test]
fn vpmuldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 690595115, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 40, 60, 133, 43, 165, 41, 41], OperandSize::Qword)
}

#[test]
fn vpmuldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 143, 40, 225], OperandSize::Dword)
}

#[test]
fn vpmuldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 137, 40, 2], OperandSize::Dword)
}

#[test]
fn vpmuldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 753448487, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 245, 156, 40, 148, 208, 39, 182, 232, 44], OperandSize::Dword)
}

#[test]
fn vpmuldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 253, 139, 40, 239], OperandSize::Qword)
}

#[test]
fn vpmuldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 173, 138, 40, 16], OperandSize::Qword)
}

#[test]
fn vpmuldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(RDX, 2104853716, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 221, 157, 40, 138, 212, 132, 117, 125], OperandSize::Qword)
}

#[test]
fn vpmuldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 40, 199], OperandSize::Dword)
}

#[test]
fn vpmuldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 172, 40, 36, 83], OperandSize::Dword)
}

#[test]
fn vpmuldq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 905105728, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 190, 40, 132, 243, 64, 209, 242, 53], OperandSize::Dword)
}

#[test]
fn vpmuldq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 178, 181, 166, 40, 203], OperandSize::Qword)
}

#[test]
fn vpmuldq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 141029036, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 205, 174, 40, 156, 138, 172, 238, 103, 8], OperandSize::Qword)
}

#[test]
fn vpmuldq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 181, 185, 40, 34], OperandSize::Qword)
}

#[test]
fn vpmuldq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 204, 40, 222], OperandSize::Dword)
}

#[test]
fn vpmuldq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 424166075, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 201, 40, 171, 187, 66, 72, 25], OperandSize::Dword)
}

#[test]
fn vpmuldq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 65426157, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 40, 132, 82, 237, 82, 230, 3], OperandSize::Dword)
}

#[test]
fn vpmuldq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 253, 195, 40, 233], OperandSize::Qword)
}

#[test]
fn vpmuldq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 213, 201, 40, 8], OperandSize::Qword)
}

#[test]
fn vpmuldq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 189, 215, 40, 28, 66], OperandSize::Qword)
}

