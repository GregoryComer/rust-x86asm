use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 246, 251], OperandSize::Dword)
}

#[test]
fn vpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1124609526, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 246, 4, 141, 246, 45, 8, 67], OperandSize::Dword)
}

#[test]
fn vpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 246, 228], OperandSize::Qword)
}

#[test]
fn vpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 246, 48], OperandSize::Qword)
}

#[test]
fn vpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 246, 214], OperandSize::Dword)
}

#[test]
fn vpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 246, 12, 78], OperandSize::Dword)
}

#[test]
fn vpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 246, 195], OperandSize::Qword)
}

#[test]
fn vpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 246, 1], OperandSize::Qword)
}

#[test]
fn vpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 246, 207], OperandSize::Dword)
}

#[test]
fn vpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 246, 36, 152], OperandSize::Dword)
}

#[test]
fn vpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 125, 8, 246, 246], OperandSize::Qword)
}

#[test]
fn vpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM26)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 45, 0, 246, 18], OperandSize::Qword)
}

#[test]
fn vpsadbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 246, 216], OperandSize::Dword)
}

#[test]
fn vpsadbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 246, 31], OperandSize::Dword)
}

#[test]
fn vpsadbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 29, 40, 246, 235], OperandSize::Qword)
}

#[test]
fn vpsadbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 77, 32, 246, 54], OperandSize::Qword)
}

#[test]
fn vpsadbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 125, 72, 246, 255], OperandSize::Dword)
}

#[test]
fn vpsadbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 101, 72, 246, 42], OperandSize::Dword)
}

#[test]
fn vpsadbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 193, 69, 72, 246, 233], OperandSize::Qword)
}

#[test]
fn vpsadbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectDisplaced(RSI, 2054071007, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 37, 64, 246, 142, 223, 162, 110, 122], OperandSize::Qword)
}

