use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 73, 66, 203, 21], OperandSize::Dword)
}

fn vmpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(104)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 81, 66, 39, 104], OperandSize::Dword)
}

fn vmpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 66, 235, 127], OperandSize::Qword)
}

fn vmpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 782332025, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(18)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 66, 156, 209, 121, 112, 161, 46, 18], OperandSize::Qword)
}

fn vmpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 66, 205, 34], OperandSize::Dword)
}

fn vmpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(30)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 101, 66, 35, 30], OperandSize::Dword)
}

fn vmpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(75)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 109, 66, 208, 75], OperandSize::Qword)
}

fn vmpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(127)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 85, 66, 36, 81, 127], OperandSize::Qword)
}

