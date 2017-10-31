use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 247], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 998581148, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 156, 65, 156, 35, 133, 59], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 222], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 51, 49], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 251], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 1673159436, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 4, 93, 12, 99, 186, 99], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 213], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1899975493, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 51, 172, 138, 69, 83, 63, 113], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 51, 226], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 624561962, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 51, 4, 77, 42, 15, 58, 37], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 125, 137, 51, 207], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 137, 51, 36, 118], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 51, 241], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 174, 51, 62], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM19)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 194, 125, 170, 51, 216], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(YMM20)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Four, 2034812895, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 171, 51, 164, 136, 223, 199, 72, 121], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 51, 219], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectDisplaced(EAX, 439907781, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 51, 168, 197, 117, 56, 26], OperandSize::Dword)
}

#[test]
fn vpmovzxwd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 51, 242], OperandSize::Qword)
}

#[test]
fn vpmovzxwd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVZXWD, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1515936947, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 51, 44, 205, 179, 92, 91, 90], OperandSize::Qword)
}

