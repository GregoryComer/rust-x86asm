use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmuldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 40, 207], OperandSize::Dword)
}

#[test]
fn vpmuldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 40, 63], OperandSize::Dword)
}

#[test]
fn vpmuldq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 40, 247], OperandSize::Qword)
}

#[test]
fn vpmuldq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RAX, 1005784708, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 40, 128, 132, 14, 243, 59], OperandSize::Qword)
}

#[test]
fn vpmuldq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 40, 226], OperandSize::Dword)
}

#[test]
fn vpmuldq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 1205071328, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 40, 178, 224, 237, 211, 71], OperandSize::Dword)
}

#[test]
fn vpmuldq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 40, 253], OperandSize::Qword)
}

#[test]
fn vpmuldq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 40, 12, 179], OperandSize::Qword)
}

#[test]
fn vpmuldq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 40, 217], OperandSize::Dword)
}

#[test]
fn vpmuldq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 735725445, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 141, 40, 52, 85, 133, 71, 218, 43], OperandSize::Dword)
}

#[test]
fn vpmuldq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EDX, 1832286872, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 159, 40, 130, 152, 122, 54, 109], OperandSize::Dword)
}

#[test]
fn vpmuldq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 194, 253, 133, 40, 202], OperandSize::Qword)
}

#[test]
fn vpmuldq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RAX, 1118129275, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 133, 142, 40, 160, 123, 76, 165, 66], OperandSize::Qword)
}

#[test]
fn vpmuldq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Eight, 1587972564, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 181, 156, 40, 132, 242, 212, 137, 166, 94], OperandSize::Qword)
}

#[test]
fn vpmuldq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 40, 227], OperandSize::Dword)
}

#[test]
fn vpmuldq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 1121791064, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 221, 170, 40, 172, 127, 88, 44, 221, 66], OperandSize::Dword)
}

#[test]
fn vpmuldq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 190, 40, 23], OperandSize::Dword)
}

#[test]
fn vpmuldq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 130, 181, 165, 40, 193], OperandSize::Qword)
}

#[test]
fn vpmuldq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 960361037, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 141, 167, 40, 44, 181, 77, 242, 61, 57], OperandSize::Qword)
}

#[test]
fn vpmuldq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 802321683, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 188, 40, 20, 125, 19, 117, 210, 47], OperandSize::Qword)
}

#[test]
fn vpmuldq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 205, 40, 253], OperandSize::Dword)
}

#[test]
fn vpmuldq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 205, 40, 34], OperandSize::Dword)
}

#[test]
fn vpmuldq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 222, 40, 60, 247], OperandSize::Dword)
}

#[test]
fn vpmuldq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 245, 195, 40, 254], OperandSize::Qword)
}

#[test]
fn vpmuldq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 157, 202, 40, 52, 193], OperandSize::Qword)
}

#[test]
fn vpmuldq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULDQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 149, 209, 40, 17], OperandSize::Qword)
}

