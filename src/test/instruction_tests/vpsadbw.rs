use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 246, 243], OperandSize::Dword)
}

fn vpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 246, 44, 144], OperandSize::Dword)
}

fn vpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 246, 214], OperandSize::Qword)
}

fn vpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 246, 28, 242], OperandSize::Qword)
}

fn vpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 246, 253], OperandSize::Dword)
}

fn vpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 246, 19], OperandSize::Dword)
}

fn vpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 246, 194], OperandSize::Qword)
}

fn vpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 570085394, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 246, 12, 221, 18, 208, 250, 33], OperandSize::Qword)
}

fn vpsadbw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 246, 219], OperandSize::Dword)
}

fn vpsadbw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 246, 14], OperandSize::Dword)
}

fn vpsadbw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM8)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 17, 61, 8, 246, 214], OperandSize::Qword)
}

fn vpsadbw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM8)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 57, 246, 3], OperandSize::Qword)
}

fn vpsadbw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 246, 215], OperandSize::Dword)
}

fn vpsadbw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 749363579, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 246, 36, 69, 123, 97, 170, 44], OperandSize::Dword)
}

fn vpsadbw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 101, 32, 246, 213], OperandSize::Qword)
}

fn vpsadbw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM15)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 133, 246, 18], OperandSize::Qword)
}

fn vpsadbw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 93, 72, 246, 224], OperandSize::Dword)
}

fn vpsadbw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EDX, 535218411, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 109, 72, 246, 178, 235, 200, 230, 31], OperandSize::Dword)
}

fn vpsadbw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 33, 85, 72, 246, 226], OperandSize::Qword)
}

fn vpsadbw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSADBW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 85, 72, 246, 52, 147], OperandSize::Qword)
}

