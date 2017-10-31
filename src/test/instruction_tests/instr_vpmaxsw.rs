use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 238, 230], OperandSize::Dword)
}

#[test]
fn vpmaxsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 1446516390, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 238, 156, 241, 166, 22, 56, 86], OperandSize::Dword)
}

#[test]
fn vpmaxsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 238, 205], OperandSize::Qword)
}

#[test]
fn vpmaxsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 2145893830, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 238, 36, 205, 198, 189, 231, 127], OperandSize::Qword)
}

#[test]
fn vpmaxsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 239], OperandSize::Dword)
}

#[test]
fn vpmaxsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ESI, 1685963301, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 238, 166, 37, 194, 125, 100], OperandSize::Dword)
}

#[test]
fn vpmaxsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 238], OperandSize::Qword)
}

#[test]
fn vpmaxsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1490738985, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 238, 52, 125, 41, 223, 218, 88], OperandSize::Qword)
}

#[test]
fn vpmaxsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 85, 137, 238, 221], OperandSize::Dword)
}

#[test]
fn vpmaxsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 117, 139, 238, 44, 143], OperandSize::Dword)
}

#[test]
fn vpmaxsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 161, 109, 141, 238, 206], OperandSize::Qword)
}

#[test]
fn vpmaxsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 1246316876, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 77, 135, 238, 60, 149, 76, 73, 73, 74], OperandSize::Qword)
}

#[test]
fn vpmaxsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 174, 238, 227], OperandSize::Dword)
}

#[test]
fn vpmaxsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 203530878, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 170, 238, 164, 179, 126, 162, 33, 12], OperandSize::Dword)
}

#[test]
fn vpmaxsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 129, 45, 170, 238, 230], OperandSize::Qword)
}

#[test]
fn vpmaxsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 531513503, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 21, 166, 238, 36, 245, 159, 64, 174, 31], OperandSize::Qword)
}

#[test]
fn vpmaxsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 205, 238, 252], OperandSize::Dword)
}

#[test]
fn vpmaxsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 93, 204, 238, 32], OperandSize::Dword)
}

#[test]
fn vpmaxsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 17, 29, 198, 238, 254], OperandSize::Qword)
}

#[test]
fn vpmaxsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 2140309555, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 21, 198, 238, 188, 207, 51, 136, 146, 127], OperandSize::Qword)
}

